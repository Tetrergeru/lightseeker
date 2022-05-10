import bpy
from mathutils import Matrix, Vector, Euler

ob = list(filter(lambda it: it.name == 'Cylinder', bpy.context.scene.objects))[0]
arm = ob.find_armature()
mesh = ob.to_mesh()
anim = arm.animation_data

def export_skl(arm, fname):
    with open(fname, "w") as file:
        for bone in arm.pose.bones:
            print(bone.name)
            
            file.write("b %s\n" % bone.name)
            if bone.parent:
                file.write("bp %s\n" % bone.parent.name)
            file.write("bb ")
            
#            print(matrix_decompose())
#            print(bone.bone.length)
            # file.write("%f " % a)
            m = bone.bone.matrix_local
            m = arm.convert_space(pose_bone=bone, 
                matrix=bone.matrix_basis,
                from_space='LOCAL',
                to_space='WORLD',
            )
            
            for a in matrix_decompose_anim(m):
                file.write("%f " % a)
            file.write("\n\n")

def export_skin(mesh, arm, fname):
    i = 1
    bones = {}
    for bone in arm.pose.bones:
#        print(bone.name, i)
        bones[bone.name] = i
        i += 1
    with open(fname, "w") as file:
        for v in mesh.vertices:
            file.write("vl %d " % (v.index + 1))
            for g in v.groups:
                if not ob.vertex_groups[g.group].name in bones:
                    print("unknown group: ", g.group)
#                if g.weight < 0.7:
#                    continue
                file.write("%s %f " % (bones[ob.vertex_groups[g.group].name], g.weight))
            file.write("\n")

def export_anim(arm, fname):
    print("export_anim")
    with open(fname, "w") as file:
        for i in range(0, 2):
            j = i * 10
            bpy.context.scene.frame_set(j)
            file.write("fr %d\n" % (j + 1))
            for bone in arm.pose.bones:
                print(bone.name)
                
                file.write("af ")
                m = bone.matrix
                
                m = arm.convert_space(pose_bone=bone, 
                    matrix=bone.matrix_basis,
                    from_space='LOCAL',
                    to_space='WORLD',
                )
                
                for a in matrix_decompose(m):
                    file.write("%f " % a)
                file.write("\n")
            file.write("\n")

EPS = 0.0001

def matrix_decompose_anim(mat):
    tr, rot, sc = mat.decompose()
    rot = rot.to_euler()#"XYZ"
    result = [
        -rot.x,
        -rot.z,
        rot.y,
    ]
    result.extend([
        tr.x,
        tr.z,
        -tr.y,
    ])
    return result

def matrix_decompose(mat):
    tr, rot, sc = mat.decompose()
    rot = rot.to_euler()#"XYZ"
    result = [
        -rot.x,
        -rot.z,
        rot.y,
    ]
    #if abs(tr[0]) > EPS or abs(tr[1]) > EPS or abs(tr[2]) > EPS:
    result.extend([
        tr.x,
        tr.z,
        -tr.y,
    ])
#    tr = Vector((-tr.x, tr.z, tr.y, 1.0))
#    rot = Euler((-rot.x, rot.z, rot.y), 'XYZ')
#    
#    m_rot = rot.to_matrix()
#    m_rot.resize_4x4() 

#    print(Matrix.Translation(tr) @ m_rot)
#    print(result)
    return result

export_skl(arm, "walk.skl")
export_skin(mesh, arm, "walk.skin")
export_anim(arm, "walk.anim")